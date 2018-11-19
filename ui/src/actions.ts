import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from './rpc';
import { timer } from 'rxjs';

let notificationId = 0;
export class Actions implements ActionsType<State, Actions> {
  hideNotificationLater = (id, delay) => (_, actions: Actions) => timer(delay).subscribe(() => actions.hideNotification(id));
  hideNotification = id => (state: State, _) => state.notificationId === id ? { notification: '' } : state;
  showNotification = (notification: string) => (_, actions: Actions) => {
    actions.hideNotificationLater(++notificationId, 2000);
    return { notification, notificationId };
  };
  getFile = () => (_, actions: Actions) => rpc.invoke('getFile').then(actions.setText);
  setText = (text: string[]) => ({ filePath: text[0], text: text[1] });
  openDialog = () => rpc.invoke('openDialog').then(actions.getFile);
}

export const actions = new Actions();
