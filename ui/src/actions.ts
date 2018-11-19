import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from './rpc';
import { timer } from 'rxjs';

let notificationId = 0;
export class Actions implements ActionsType<State, Actions> {
  hideNotificationLater = (v: { id: number; delay: number }) => (_, actions: Actions) =>
    timer(v.delay).subscribe(() => actions.hideNotification(v.id));
  hideNotification = id => (state: State, _) => (state.notificationId === id ? { notification: '' } : state);
  showNotification = (notification: string) => (_, actions: Actions) => {
    actions.hideNotificationLater({ id: ++notificationId, delay: 5000 });
    return { notification, notificationId };
  };
  getFile = () => (_, actions: Actions) => rpc.invoke('getFile').then(actions.setText);
  setText = (text: string[]) => ({ filePath: text[0], text: text[1] });
  openDialog = () => rpc.invoke('openDialog').then(actions.getFile);
}

export const actions = new Actions();
