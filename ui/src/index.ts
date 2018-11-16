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
  notification: '',
  notificationShown: false,
  filePath: '',
  text: ''
};

withLogger(app)(initialState, actions, view, document.body);


// const config = { devtools: 'hyperapp-redux-devtools'};
// let main;
// if (!config.devtools || config.devtools === 'none') main = withLogger(app)(initialState, actions, view, document.body);
// else
//   switch (config.devtools) {
//     case 'hyperapp-redux-devtools':
//       import('hyperapp-redux-devtools').then(
//         devtools => (main = withLogger(devtools.default(app))(initialState, actions, view, document.body))
//       );
//       break;
//     case 'hyperapp-devtools':
//       import('hyperapp-devtools').then(
//         devtools => (main = withLogger(devtools.default(app))(initialState, actions, view, document.body))
//       );
//       break;
//   }
