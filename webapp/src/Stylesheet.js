import React from 'react';
import { Global, css } from '@emotion/core';
import { lighten } from 'polished';

import 'normalize.css';

export const WHITE = '#FFFFFF';
export const BLACK = '#000000';
export const PRIMARY = '#f77976';
export const GREY = '#808080';
export const DARK_GREY = '#505050';
export const PEDESTRIAN_1 = '#ff0000';
export const PEDESTRIAN_2 = '#0000ff';

const Stylesheet = () => (
  <Global
    styles={css`
      html {
        // Border box
        *,
        *::before,
        *::after {
          box-sizing: border-box;
        }
      }
      body {
        min-height: 100vh;
      }

      a,
      button {
        color: inherit;
        cursor: pointer;
        background: none;
        border: none;
        &:active,
        &:focus {
          color: ${lighten(0.05, PRIMARY)};
          outline: none;
        }
        &:hover {
          color: ${PRIMARY};
        }
        &:disabled {
          filter: opacity(50%);
          &:hover {
            cursor: not-allowed;
          }
        }
      }
    `}
  />
);

export default Stylesheet;
