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
  .then(({ Universe }) => {
    const universe = Universe.new();
    universe.load_scenario(
      // JSON.stringify({
      //   scenario: 'AntipodalCircle',
      //   agents_count: 9,
      //   radius: 6,
      // })
      JSON.stringify({
        scenario: 'Corridor',
        agents_per_side_count: 1,
        length: 15,
        width: 1.5,
      })
    );
    ReactDOM.render(
      <React.StrictMode>
        <App universe={universe} />
      </React.StrictMode>,
      document.getElementById('root')
    );
  })
  .catch((e) => console.error('Error importing `index.js`:', e));
