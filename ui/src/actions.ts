import State from './state';
import { ActionsType } from 'hyperapp';
import { rpc } from "./rpc";

export class Actions implements ActionsType<State, Actions> {
  getFile = (filePath: string) => (state, actions: Actions) => {
    rpc.invoke('getFile', { filePath }).then(actions.setText);
    return { filePath };
  };
  setText = text => state => ({ text });
  openDialog = () => state => { rpc.invoke('openDialog'); return state };
}

export const actions = new Actions();








