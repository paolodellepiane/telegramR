import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from './rpc';
import { timer } from 'rxjs';
import * as _ from 'lodash';

export class Actions implements ActionsType<State, Actions> {
  hideNotification = id => (state: State, __) => (_.get(state, 'notification.id') === id ? { notification: null } : null);
  showNotification = (text: string) => (state, actions: Actions): Partial<State> => {
    const id = _.get(state, 'notification.id', 0) + 1;
    timer(5000).subscribe(() => actions.hideNotification(id));
    return { notification: { id, text } };
  };
  getFile = () => (_, actions: Actions) => rpc.invoke('getFile').then(actions.setText);
  setText = (text: string[]) => ({ filePath: text[0], text: text[1] });
  openDialog = () => rpc.invoke('openDialog').then(actions.getFile);
}

export const actions = new Actions();
