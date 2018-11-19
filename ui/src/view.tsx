import { h, View } from 'hyperapp';
import State from './state';
import { Actions } from './actions';

const Notification = ({ message }) => <div class={'notification is-success hidden-div ' + (message && ' show')}>{message}</div>;

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
    <Notification message={state.notification} />
  </main>
);
