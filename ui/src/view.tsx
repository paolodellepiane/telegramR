import { h, View } from 'hyperapp';
import State from './state';
import { Actions } from './actions';
import { Enter, Exit } from '@hyperapp/transitions';

const Notification = ({ message, oncreate }) => (
  <Enter css={{ opacity: '0' }}>
    <Exit css={{ opacity: '0', transform: 'scale(2.0,2.0)' }}>
      <div class="notification is-success" style="{{ height: '100px' }}" oncreate={oncreate}>
        {message}
      </div>
    </Exit>
  </Enter>
);

export const view: View<State, Actions> = (state, actions) => (
  <main>
    <div class="container">
      <textarea class="textarea" placeholder="10 lines of textarea" rows="10" value={state.text} />
      <div style={{ display: 'flex', justifyContent: 'center' }}>
        <button style={{ margin: '5px 5px' }} class="button is-dark" onclick={_ => actions.getFile()}>
          open
        </button>
        <button style={{ margin: '5px 5px' }} class="button is-dark" onclick={_ => actions.setText(['', ''])}>
          reset
        </button>
        <button style={{ margin: '5px 5px' }} class="button is-success" onclick={_ => actions.showNotification('blah')}>
          notify
        </button>
      </div>
    </div>
    {state.notificationShown && <Notification message={state.notification} oncreate={_ => actions.hideNotificationLater(1000)}/>}
  </main>
);
