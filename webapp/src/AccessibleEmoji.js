import React from 'react';

const AccessibleEmoji = ({ emoji, label }) => (
  <span role="img" label={label}>
    {emoji}
  </span>
);

export default AccessibleEmoji;
