import React, { useState, useEffect, Suspense } from 'react';
import { PEDESTRIAN_1, PEDESTRIAN_2 } from './Stylesheet';
import { mix } from 'polished';
import { Vector3 } from 'three';
import { useGLTF } from '@react-three/drei';
import DebugCircle from './DebugCircle';

const COLOR_VECTOR = new Vector3(1, 0, 0.5).normalize();
const COLOR_EXTENT = 10;

const PEDESTRIAN_URL = `${process.env.PUBLIC_URL}/assets/pedestrian.gltf`;

const FallbackPedestrianMesh = ({ radius, height, color, ...otherProps }) => (
  <mesh position={[0, height / 2, 0]} {...otherProps}>
    <cylinderBufferGeometry args={[radius, radius, height, 20]} />
    <meshStandardMaterial color={color} />
  </mesh>
);

const PedestrianMesh = ({ radius, height, color, ...otherProps }) => {
  const { nodes } = useGLTF(PEDESTRIAN_URL);
  return (
    <mesh
      geometry={nodes.Pedestrian.geometry}
      scale={[radius, height, radius]}
      rotation={[0, -Math.PI / 2, 0]}
      {...otherProps}
    >
      <meshStandardMaterial color={color} />
    </mesh>
  );
};

const PedestrianMeshWithFallback = (props) => (
  <Suspense fallback={<FallbackPedestrianMesh {...props} />}>
    <PedestrianMesh {...props} />
  </Suspense>
);

const Pedestrian = ({
  position,
  direction,
  radius,
  selected,
  height = 2,
  onClick,
}) => {
  const [color, setColor] = useState(PEDESTRIAN_1);
  // Computing the color from the initial pedestrian position.
  useEffect(
    () => {
      const p = new Vector3(position.x, 0, position.y);
      const alpha =
        Math.max(-0.5, Math.min(0.5, p.dot(COLOR_VECTOR) / COLOR_EXTENT)) + 0.5;
      setColor(mix(alpha, PEDESTRIAN_1, PEDESTRIAN_2));
    }, // eslint-disable-next-line
    []
  );
  return (
    <group
      position={[position.x, 0, position.y]}
      rotation={[0, Math.atan2(-direction.y, direction.x), 0]}
    >
      <PedestrianMeshWithFallback
        radius={radius}
        height={height}
        color={color}
        castShadow
        onClick={onClick}
      />
      {selected ? (
        <DebugCircle radius={radius} size={0.05} color="#4af2a1" outerGlow />
      ) : null}
    </group>
  );
};

export default Pedestrian;
