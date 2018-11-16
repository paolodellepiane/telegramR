import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from './rpc';

export class Actions implements ActionsType<State, Actions> {
  hideNotificationLater = delay => setTimeout(() => this.hideNotification(), delay);
  hideNotification = () => ({ notificationShown: false });
  showNotification = (text: string) => ({ notificationShown: true, notification: text });
  getFile = () => (state, actions: Actions) => {
    rpc.invoke('getFile').then(actions.setText);
    return state;
  };
  setText = (text: string[]) => state => ({ filePath: text[0], text: text[1] });
  openDialog = () => state => {
    rpc.invoke('openDialog').then(actions.getFile);
    return state;
  };
}

export const actions = new Actions();
