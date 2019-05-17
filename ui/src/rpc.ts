import { Subject } from 'rxjs';
import { take } from 'rxjs/operators';

export class Rpc {
  public readonly messages = new Subject<string>();
  public invoke: (action: string, data?: any) => Promise<any>;
  constructor(protocol: string) {
    console.log('[RPC] Setting protocol ' + protocol);
    switch (protocol.toLowerCase()) {
      case 'ws':
        this.invoke = this.getWsInvoke() || ((_, __) => Promise.resolve(null));
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
      (window.external as any).invoke(JSON.stringify({ action, ...data }));
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
    } catch {
      return null;
    }
  }
}

console.log('env.RPC: ' + process.env.RPC);
export const rpc = new Rpc(process.env.RPC || (process.env.NODE_ENV === 'production' ? 'interop' : 'ws'));
