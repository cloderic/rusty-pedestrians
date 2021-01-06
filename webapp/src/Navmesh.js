import React, { useMemo } from 'react';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader';

const Navmesh = ({ navmeshObj, color }) => {
  const navmesh = useMemo(() => {
    const loader = new OBJLoader();
    const scene = loader.parse(navmeshObj);
    return scene.children[0];
  }, [navmeshObj]);

  return (
    <primitive
      object={navmesh}
      position={[0, 0, 0]}
      rotation={[-Math.PI / 2, 0, 0]}
      receiveShadow
    >
      <meshStandardMaterial color={color} />
    </primitive>
  );
};
export default Navmesh;
