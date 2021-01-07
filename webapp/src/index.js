import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import * as serviceWorker from './serviceWorker';

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();

// A dependency graph that contains any wasm must all be imported
// asynchronously.
import('rusty-pedestrians-engine')
  .then((engine) => {
    const universe = engine.Universe.new();
    ReactDOM.render(
      <React.StrictMode>
        <App universe={universe} />
      </React.StrictMode>,
      document.getElementById('root')
    );
  })
  .catch((e) =>
    console.error('Error importing `rusty-pedestrians-engine`: ', e)
  );
