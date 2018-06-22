import { Subject } from 'rxjs';
import { take } from 'rxjs/operators';

export class Rpc {
  public readonly messages = new Subject<string>();
  public invoke: (action: string, data?: any) => Promise<any>;
  constructor(protocol: string) {
    switch (protocol.toLowerCase()) {
      case 'ws':
        this.invoke = this.getWsInvoke();
        break;
      default:
        this.invoke = this.getInteropInvoke();
        break;
    }
  }
  getInteropInvoke() {
    window['render'] = ev => setTimeout(() => this.messages.next(ev));
    return (action: string, data: any = {}): Promise<any> => {
      if (typeof data !== 'object') throw Error('invoke: data must be an object');
      window.external.invoke(JSON.stringify({ action, ...data }));
      return this.messages.pipe(take(1)).toPromise();
    };
  }
  getWsInvoke() {
    try {
      const url = 'ws://127.0.0.1:36767';
      let ws = new WebSocket(url);
      ws.onmessage = ev => this.messages.next(JSON.parse(ev.data));
      return (action: string, data: any = {}): Promise<any> => {
        if (typeof data !== 'object') throw Error('invoke: data must be an object');
        if (!ws || ws.readyState !== ws.OPEN) ws = new WebSocket(url);
        ws.send(JSON.stringify({ action, ...data }));
        return this.messages.pipe(take(1)).toPromise();
      };
    } catch {}
  }
}

export let rpc = new Rpc('ws');
window['setRpc'] = (protocol: string) => {
  rpc = new Rpc(protocol);
};
