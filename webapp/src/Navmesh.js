import React, { useMemo } from 'react';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader';
import { MeshBasicMaterial } from 'three';

const Navmesh = ({ navmeshObj, color }) => {
  const navmesh = useMemo(() => {
    const loader = new OBJLoader();
    const scene = loader.parse(navmeshObj);
    return scene.children[0];
  }, [navmeshObj]);
  const material = useMemo(() => new MeshBasicMaterial({ color }), [color]);

  return (
    <primitive
      object={navmesh}
      position={[0, 0, 0]}
      rotation={[-Math.PI / 2, 0, 0]}
      material={material}
      receiveShadow
    />
  );
};
export default Navmesh;
