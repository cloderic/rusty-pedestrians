import React, { useMemo } from 'react';
import { CanvasTexture, Color } from 'three';

const createAlphaMap = (size = 512) => {
  const canvas = document.createElement('canvas');
  canvas.width = size;
  canvas.height = size;

  const ctx = canvas.getContext('2d');

  // Create a gradient
  const grd = ctx.createLinearGradient(0, size / 2, size, size / 2);
  grd.addColorStop(0, 'white');
  grd.addColorStop(1, 'black');

  // Create a rectangle filled with the gradient
  ctx.fillStyle = grd;
  ctx.fillRect(0, 0, size, size);

  return new CanvasTexture(canvas);
};

const DebugHalfplane = ({
  origin,
  altitude,
  direction,
  color,
  width = 0.5,
  length = 512,
}) => {
  const alphaMap = useMemo(() => createAlphaMap(), []);
  return (
    <group
      position={[origin.x, altitude, origin.y]}
      rotation={[0, Math.atan2(-direction.y, direction.x), 0]}
    >
      <mesh
        position={[0, 0, -width / 2]}
        rotation={[-Math.PI / 2, 0, Math.PI / 2]}
      >
        <planeBufferGeometry args={[width, length]} />
        <meshBasicMaterial
          color={new Color(color)}
          alphaMap={alphaMap}
          transparent
        />
      </mesh>
    </group>
  );
};

export default DebugHalfplane;
