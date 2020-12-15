(this["webpackJsonprusty-pedestrians-webapp"]=this["webpackJsonprusty-pedestrians-webapp"]||[]).push([[0],{71:function(e,t,n){e.exports=n(84)},84:function(e,t,n){"use strict";n.r(t);var a=n(1),r=n.n(a),o=n(29),c=n.n(o),i=n(8),l=n(38),s=n(9),u=n(57),d=n.n(u),m=n(55),f=n(35),h=n(47);n(75);function b(){var e=Object(l.a)(["\n      html {\n        // Border box\n        *,\n        *::before,\n        *::after {\n          box-sizing: border-box;\n        }\n      }\n      body {\n        min-height: 100vh;\n      }\n\n      a,\n      button {\n        color: inherit;\n        cursor: pointer;\n        background: none;\n        border: none;\n        &:active,\n        &:focus {\n          color: ",";\n          outline: none;\n        }\n        &:hover {\n          color: ",";\n        }\n        &:disabled {\n          filter: opacity(50%);\n          &:hover {\n            cursor: not-allowed;\n          }\n        }\n      }\n    "]);return b=function(){return e},e}var g=function(){return r.a.createElement(f.a,{styles:Object(f.c)(b(),Object(h.a)(.05,"#f77976"),"#f77976")})},p=n(0),E=n(30),v=function(e,t){var n=arguments.length>2&&void 0!==arguments[2]?arguments[2]:512,a=n/(2*Math.max(e,t)),r=[e*a,t*a],o=document.createElement("canvas");o.width=n,o.height=n;var c=o.getContext("2d"),i=c.createRadialGradient(n/2,n/2,r[0],n/2,n/2,r[1]);return i.addColorStop(0,"white"),i.addColorStop(1,"black"),c.fillStyle=i,c.fillRect(0,0,n,n),new p.CanvasTexture(o)},w=function(e){var t=e.radius,n=e.color,o=e.outerGlow,c=void 0!==o&&o,i=e.width,l=void 0===i?.15:i,s=e.segments,u=void 0===s?100:s,d=Object(a.useMemo)((function(){return v(t,c?t+l:t-l)}),[t,l,c]);return r.a.createElement("mesh",{rotation:[-Math.PI/2,0,0]},c?r.a.createElement("ringBufferGeometry",{args:[t,t+l,u]}):r.a.createElement("ringBufferGeometry",{args:[t-l,t,u]}),r.a.createElement("meshBasicMaterial",{color:new p.Color(n),alphaMap:d,transparent:!0}))},j=new p.Vector3(1,0,.5).normalize(),y="".concat("/rusty-pedestrians","/assets/pedestrian.gltf"),O=function(e){var t=e.radius,n=e.height,a=e.color,o=Object(m.a)(e,["radius","height","color"]);return r.a.createElement("mesh",Object.assign({position:[0,n/2,0]},o),r.a.createElement("cylinderBufferGeometry",{args:[t,t,n,20]}),r.a.createElement("meshStandardMaterial",{color:a}))},C=function(e){var t=e.radius,n=e.height,a=e.color,o=Object(m.a)(e,["radius","height","color"]),c=Object(E.d)(y).nodes;return r.a.createElement("mesh",Object.assign({geometry:c.Pedestrian.geometry,scale:[t,n,t],rotation:[0,-Math.PI/2,0]},o),r.a.createElement("meshStandardMaterial",{color:a}))},x=function(e){return r.a.createElement(a.Suspense,{fallback:r.a.createElement(O,e)},r.a.createElement(C,e))},k=function(e){var t=e.position,n=e.direction,o=e.radius,c=e.selected,l=e.height,s=void 0===l?2:l,u=e.onClick,d=Object(a.useState)("#ff0000"),m=Object(i.a)(d,2),f=m[0],b=m[1];return Object(a.useEffect)((function(){var e=new p.Vector3(t.x,0,t.y),n=Math.max(-.5,Math.min(.5,e.dot(j)/10))+.5;b(Object(h.b)(n,"#ff0000","#0000ff"))}),[]),r.a.createElement("group",{position:[t.x,0,t.y],rotation:[0,Math.atan2(-n.y,n.x),0]},r.a.createElement(x,{radius:o,height:s,color:f,castShadow:!0,onClick:u}),c?r.a.createElement(w,{radius:o,size:.05,color:"#4af2a1",outerGlow:!0}):null)},S=function(e){var t=e.origin,n=e.altitude,o=e.direction,c=e.color,i=e.width,l=void 0===i?.5:i,s=e.length,u=void 0===s?512:s,d=Object(a.useMemo)((function(){return function(){var e=arguments.length>0&&void 0!==arguments[0]?arguments[0]:512,t=document.createElement("canvas");t.width=e,t.height=e;var n=t.getContext("2d"),a=n.createLinearGradient(0,e/2,e,e/2);return a.addColorStop(0,"white"),a.addColorStop(1,"black"),n.fillStyle=a,n.fillRect(0,0,e,e),new p.CanvasTexture(t)}()}),[]);return r.a.createElement("group",{position:[t.x,n,t.y],rotation:[0,Math.atan2(-o.y,o.x),0]},r.a.createElement("mesh",{position:[0,0,-l/2],rotation:[-Math.PI/2,0,Math.PI/2]},r.a.createElement("planeBufferGeometry",{args:[l,u]}),r.a.createElement("meshBasicMaterial",{color:new p.Color(c),alphaMap:d,transparent:!0})))},M=new p.Color("#0068FF"),B=new p.Color("#310fb8"),G=new p.Color("#0068FF"),I=new p.Color("#39B92C"),P=new p.Color("#ff6442"),_=function(e){var t=e.agent,n=e.orca_constraints;return r.a.createElement("group",null,r.a.createElement("group",{position:[t.position.x,0,t.position.y]},n.map((function(e,t){var n=Object(i.a)(e,2),a=n[0],o=n[1];return r.a.createElement(S,{key:t,altitude:.05*t,origin:a,direction:o,color:P})})),r.a.createElement("group",{position:[0,.05*n.length,0]},r.a.createElement(E.a,{points:[[0,0,0],[t.velocity.x,0,t.velocity.y]],color:M}),r.a.createElement(w,{radius:t.maximum_speed,color:B}),r.a.createElement(w,{radius:t.desired_speed,color:G}))),r.a.createElement("group",{position:[t.target.x,0,t.target.y]},r.a.createElement("mesh",null,r.a.createElement("sphereBufferGeometry",{args:[.1]}),r.a.createElement("meshStandardMaterial",{color:I}))))},z=function(e){var t=e.color,n=Object(s.g)().scene;return Object(a.useEffect)((function(){n.background=new p.Color(t)}),[n,t]),r.a.createElement(r.a.Fragment,null,r.a.createElement("ambientLight",null),r.a.createElement("directionalLight",{castShadow:!0,position:[25,50,-8],"shadow-mapSize-width":2048,"shadow-mapSize-height":2048,"shadow-camera-far":100,"shadow-camera-left":-10,"shadow-camera-right":10,"shadow-camera-top":10,"shadow-camera-bottom":-10}))},F=n(54),J=function(e){var t=e.emoji,n=e.label;return r.a.createElement("span",{role:"img",label:n},t)};function R(){var e=Object(l.a)(["\n  font-size: 2rem;\n  display: flex;\n  justify-content: center;\n"]);return R=function(){return e},e}function L(){var e=Object(l.a)(["\n  height: 100vh;\n  background: ",";\n  display: flex;\n  flex-direction: column;\n"]);return L=function(){return e},e}Object(E.c)();var D=F.a.div(L(),"#808080"),N=F.a.div(R()),T=function(e){var t=e.universe,n=Object(a.useState)(null),o=Object(i.a)(n,2),c=o[0],l=o[1],u=Object(a.useCallback)((function(){l(null)}),[l]),m=Object(a.useState)({agents:[]}),f=Object(i.a)(m,2),h=f[0],b=h.agents,p=h.agentDebugInfo,v=f[1],w=Object(a.useCallback)((function(){var e=d()(t.render(),5).map((function(e,t){var n=Object(i.a)(e,5),a=n[0],r=n[1],o=n[2],c=n[3],s=n[4];return{index:t,position:{x:a,y:r},direction:{x:o,y:c},radius:s,handleClick:function(){l(t)}}}));v(null!=c?{agents:e,agentDebugInfo:JSON.parse(t.render_debug_info(c))}:{agents:e})}),[v,t,c,l]),j=Object(a.useState)(!1),y=Object(i.a)(j,2),O=y[0],C=y[1],x=Object(a.useCallback)((function(){t.reset(),w(),C(!0)}),[t,w]);Object(a.useEffect)((function(){t.reset()}),[t]);var S=function(){var e=arguments.length>0&&void 0!==arguments[0]&&arguments[0],t=Object(a.useState)(e),n=Object(i.a)(t,2),r=n[0],o=n[1],c=Object(a.useCallback)((function(){o((function(e){return!e}))}),[]);return[r,c]}(!0),M=Object(i.a)(S,2),B=M[0],G=M[1],I=Object(a.useCallback)((function(){t.update(1/60),C(!1),w()}),[t,w]);return Object(a.useEffect)((function(){if(!B){var e=setInterval(I,1e3/60);return function(){return clearInterval(e)}}w()}),[t,B,I,w]),r.a.createElement(r.a.Fragment,null,r.a.createElement(g,null),r.a.createElement(D,null,r.a.createElement(s.a,{camera:{position:[0,80,0],fov:10},shadowMap:!0,onPointerMissed:u},r.a.createElement(z,{color:"#808080"}),b.map((function(e){var t=e.index,n=e.position,a=e.direction,o=e.radius,i=e.handleClick;return r.a.createElement(k,{key:t,position:n,direction:a,radius:o,onClick:i,selected:t===c})})),p?r.a.createElement(_,p):null,r.a.createElement("mesh",{position:[0,0,0],rotation:[-Math.PI/2,0,0],receiveShadow:!0},r.a.createElement("planeBufferGeometry",{args:[100,100,1e3,1e3]}),r.a.createElement("shadowMaterial",{transparent:!0,opacity:.4})),r.a.createElement(E.b,null)),r.a.createElement(N,null,r.a.createElement("button",{onClick:G,name:"toggle-play-pause"},B?r.a.createElement(J,{emoji:"\u25b6\ufe0f",label:"Play"}):r.a.createElement(J,{emoji:"\u23f8",label:"Pause"})),r.a.createElement("button",{onClick:I,name:"single-step",disabled:!B},r.a.createElement(J,{emoji:"\u23ed",label:"Single Step"})),r.a.createElement("button",{onClick:x,name:"reset",disabled:O},r.a.createElement(J,{emoji:"\u21a9\ufe0f",label:"Restart"})))))};Boolean("localhost"===window.location.hostname||"[::1]"===window.location.hostname||window.location.hostname.match(/^127(?:\.(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}$/));"serviceWorker"in navigator&&navigator.serviceWorker.ready.then((function(e){e.unregister()})).catch((function(e){console.error(e.message)})),n.e(3).then(n.bind(null,87)).then((function(e){var t=e.Universe.new();t.load_scenario(JSON.stringify({scenario:"AntipodalCircle",agents_count:5,radius:6.5})),c.a.render(r.a.createElement(r.a.StrictMode,null,r.a.createElement(T,{universe:t})),document.getElementById("root"))})).catch((function(e){return console.error("Error importing `index.js`:",e)}))}},[[71,1,2]]]);
//# sourceMappingURL=main.2aa8eadb.chunk.js.map