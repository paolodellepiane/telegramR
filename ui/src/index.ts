import 'core-js';
import 'whatwg-fetch';
import '../styles/main.scss';
import { app } from 'hyperapp';
import { view } from './view';
import { actions } from './actions';
import State from './state';
import { withLogger } from '@hyperapp/logger';
import './rpc';

export const initialState: State = {
  filePath: '',
  text: ''
};

const isIE = /*@cc_on!@*/ false || !!document['documentMode'];
if (isIE) console.log('IE detected');
if (process.env.NODE_ENV === 'production') withLogger(app)(initialState, actions, view, document.body);
else {
  const config = { devtools: 'hyperapp-redux-devtools' };
  let main;
  if (!config.devtools || config.devtools === 'none') main = withLogger(app)(initialState, actions, view, document.body);
  else if (isIE || config.devtools === 'hyperapp-devtools') {
    console.log('enable hyperapp-devtools');
    import('hyperapp-devtools').then(devtools => (main = withLogger(devtools.default(app))(initialState, actions, view, document.body)));
  } else if (config.devtools === 'hyperapp-redux-devtools') {
    // import('hyperapp-redux-devtools').then(devtools => withLogger(devtools.default(app))(initialState, actions, view, document.body));
  }
}
