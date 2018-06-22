import { h, View } from 'hyperapp';
import State from './state';
import { Actions } from './actions';

export const view: View<State, Actions> = (state, actions) => (
  <main>
    <section class="section">
      <div class="container">
        <textarea class="textarea" placeholder="10 lines of textarea" rows="10" />
        <button style={{ margin: '5px 5px' }} class="button is-primary is-rounded" onclick={_ => actions.getFile()}>
          open
        </button>
        <button style={{ margin: '5px 5px' }} class="button is-primary is-rounded" onclick={_ => actions.setText(['', ''])}>
          reset
        </button>
      </div>
    </section>
  </main>
);
