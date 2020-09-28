import React from 'react';
import DebugCircle from './DebugCircle';
import DebugHalfplane from './DebugHalfplane';
import { Line } from '@react-three/drei';
import { Color } from 'three';

const VELOCITY_COLOR = new Color('#0068FF');
const MAX_SPEED_COLOR = new Color('#310fb8');
const DESIRED_SPEED_COLOR = new Color('#0068FF');
const TARGET_COLOR = new Color('#39B92C');
const CONSTRAINTS_COLOR = new Color('#ff6442');

const PedestrianDebugInfo = ({ agent, orca_constraints }) => {
  return (
    <group>
      <group position={[agent.position.x, 0, agent.position.y]}>
        <Line
          points={[
            [0, 0, 0],
            [agent.velocity.x, 0, agent.velocity.y],
          ]}
          color={VELOCITY_COLOR}
        />
        <DebugCircle radius={agent.maximum_speed} color={MAX_SPEED_COLOR} />
        <DebugCircle radius={agent.desired_speed} color={DESIRED_SPEED_COLOR} />
        {orca_constraints.map(([origin, direction]) => (
          <>
            <DebugHalfplane
              origin={origin}
              direction={direction}
              color={CONSTRAINTS_COLOR}
            />
            {/* <Line
              points={[
                [origin.x, 0, origin.y],
                [origin.x + direction.x, 0, origin.y + direction.y],
              ]}
              color={CONSTRAINTS_COLOR}
              lineWidth={2}
            />
            <group position={[origin.x, 0, origin.y]}>
              <mesh>
                <sphereBufferGeometry args={[0.1]} />
                <meshStandardMaterial color={CONSTRAINTS_COLOR} />
              </mesh>
            </group> */}
          </>
        ))}
      </group>
      <group position={[agent.target.x, 0, agent.target.y]}>
        <mesh>
          <sphereBufferGeometry args={[0.1]} />
          <meshStandardMaterial color={TARGET_COLOR} />
        </mesh>
      </group>
    </group>
  );
};

export default PedestrianDebugInfo;
