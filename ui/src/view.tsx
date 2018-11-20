import { h, View } from 'hyperapp';
import { Actions } from './actions';
import { Monaco } from './monaco';
import State from './state';
import * as _ from 'lodash';

const Notification = ({ message }) => (
  <div style={{ margin: '15px' }} class={'notification is-success hidden-div ' + (message && ' show')}>
    {message}
  </div>
);

export const view: View<State, Actions> = (state: State, actions: Actions) => (
  <main style={{ display: 'flex', height: '100%' }}>
    <div style={{ display: 'flex', flexDirection: 'column', width: '100%', margin: '10px' }}>
      <Monaco value={state.text} onchange={v => actions.setText([state.filePath, v])} />
      <div style={{ flex: '0', display: 'flex', justifyContent: 'center', minHeight: '46px' }}>
        <button style={{ margin: '5px' }} class="button is-primary" onclick={_ => actions.getFile()}>
          open
        </button>
        <button style={{ margin: '5px' }} class="button is-primary" onclick={_ => actions.setText(['', ''])}>
          reset
        </button>
        <button style={{ margin: '5px' }} class="button is-success" onclick={_ => actions.showNotification('notification')}>
          notify
        </button>
      </div>
    </div>
    <Notification message={_.get(state, 'notification.text')} />
  </main>
);
