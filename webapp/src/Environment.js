import React, { useEffect } from 'react';
import { useThree } from 'react-three-fiber';
import { Color } from 'three';

const Environment = ({ color }) => {
  const { scene } = useThree();
  useEffect(() => {
    scene.background = new Color(color);
  }, [scene, color]);
  return (
    <>
      <ambientLight />
      <directionalLight
        castShadow
        position={[25, 50, -8]}
        shadow-mapSize-width={2048}
        shadow-mapSize-height={2048}
        shadow-camera-far={100}
        shadow-camera-left={-10}
        shadow-camera-right={10}
        shadow-camera-top={10}
        shadow-camera-bottom={-10}
      />
    </>
  );
};

export default Environment;
