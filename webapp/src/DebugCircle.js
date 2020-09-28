import React, { useMemo } from 'react';
import { CanvasTexture, Color } from 'three';

const createAlphaMap = (startRadius, stopRadius, size = 512) => {
  const scale = size / (Math.max(startRadius, stopRadius) * 2);
  const scaledRadii = [startRadius * scale, stopRadius * scale];

  const canvas = document.createElement('canvas');
  canvas.width = size;
  canvas.height = size;

  const ctx = canvas.getContext('2d');

  // Create a gradient
  const grd = ctx.createRadialGradient(
    size / 2,
    size / 2,
    scaledRadii[0],
    size / 2,
    size / 2,
    scaledRadii[1]
  );
  grd.addColorStop(0, 'white');
  grd.addColorStop(1, 'black');

  // Create a rectangle filled with the gradient
  ctx.fillStyle = grd;
  ctx.fillRect(0, 0, size, size);

  return new CanvasTexture(canvas);
};

const DebugCircle = ({
  radius,
  color,
  outerGlow = false,
  width = 0.15,
  segments = 100,
}) => {
  const alphaMap = useMemo(
    () =>
      outerGlow
        ? createAlphaMap(radius, radius + width)
        : createAlphaMap(radius, radius - width),
    [radius, width, outerGlow]
  );
  return (
    <mesh rotation={[-Math.PI / 2, 0, 0]}>
      {outerGlow ? (
        <ringBufferGeometry args={[radius, radius + width, segments]} />
      ) : (
        <ringBufferGeometry args={[radius - width, radius, segments]} />
      )}
      <meshBasicMaterial
        color={new Color(color)}
        alphaMap={alphaMap}
        transparent
      />
    </mesh>
  );
};

export default DebugCircle;
