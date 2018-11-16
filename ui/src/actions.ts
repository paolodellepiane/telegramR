import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from './rpc';
import { timer } from 'rxjs';

export class Actions implements ActionsType<State, Actions> {
  hideNotificationLater = delay => (_, actions: Actions) => timer(1000).subscribe(actions.hideNotification);
  hideNotification = () => ({ notificationShown: false });
  showNotification = (text: string) => ({ notificationShown: true, notification: text });
  getFile = () => (_, actions: Actions) => rpc.invoke('getFile').then(actions.setText);
  setText = (text: string[]) => ({ filePath: text[0], text: text[1] });
  openDialog = () => rpc.invoke('openDialog').then(actions.getFile);
}

export const actions = new Actions();
