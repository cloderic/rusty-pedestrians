(this["webpackJsonprusty-pedestrians-webapp"]=this["webpackJsonprusty-pedestrians-webapp"]||[]).push([[3],{90:function(n,e,r){"use strict";(function(n){r.d(e,"a",(function(){return j})),r.d(e,"b",(function(){return x})),r.d(e,"d",(function(){return A})),r.d(e,"e",(function(){return O})),r.d(e,"c",(function(){return T})),r.d(e,"f",(function(){return E})),r.d(e,"g",(function(){return D}));var t=r(2),u=r(7),i=r(91),a=new Array(32).fill(void 0);function c(n){return a[n]}a.push(void 0,null,!0,!1);var o=a.length;function f(n){var e=c(n);return function(n){n<36||(a[n]=o,o=n)}(n),e}var l=new("undefined"===typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});l.decode();var d=null;function v(){return null!==d&&d.buffer===i.g.buffer||(d=new Uint8Array(i.g.buffer)),d}function s(n,e){return l.decode(v().subarray(n,n+e))}var b=0,p=new("undefined"===typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder)("utf-8"),h="function"===typeof p.encodeInto?function(n,e){return p.encodeInto(n,e)}:function(n,e){var r=p.encode(n);return e.set(r),{read:n.length,written:r.length}};function y(n,e,r){if(void 0===r){var t=p.encode(n),u=e(t.length);return v().subarray(u,u+t.length).set(t),b=t.length,u}for(var i=n.length,a=e(i),c=v(),o=0;o<i;o++){var f=n.charCodeAt(o);if(f>127)break;c[a+o]=f}if(o!==i){0!==o&&(n=n.slice(o)),a=r(a,i,i=o+3*n.length);var l=v().subarray(a+o,a+i);o+=h(n,l).written}return b=o,a}var _=null;function g(){return null!==_&&_.buffer===i.g.buffer||(_=new Int32Array(i.g.buffer)),_}var w=null;function k(n,e){return(null!==w&&w.buffer===i.g.buffer||(w=new Float64Array(i.g.buffer)),w).subarray(n/8,n/8+e)}var j=function(){function n(){Object(t.a)(this,n)}return Object(u.a)(n,[{key:"free",value:function(){var n=this.ptr;this.ptr=0,i.a(n)}}]),n}(),x=function(){function n(){Object(t.a)(this,n)}return Object(u.a)(n,[{key:"free",value:function(){var n=this.ptr;this.ptr=0,i.b(n)}},{key:"load_scenario",value:function(n){var e=y(n,i.e,i.f),r=b;i.i(this.ptr,e,r)}},{key:"update",value:function(n){i.n(this.ptr,n)}},{key:"render_agents",value:function(){try{var n=i.c.value-16;i.c.value=n,i.k(n,this.ptr);var e=g()[n/4+0],r=g()[n/4+1],t=k(e,r).slice();return i.d(e,8*r),t}finally{i.c.value+=16}}},{key:"render_navmesh",value:function(){try{var n=i.c.value-16;i.c.value=n,i.m(n,this.ptr);var e=g()[n/4+0],r=g()[n/4+1];return s(e,r)}finally{i.c.value+=16,i.d(e,r)}}},{key:"render_debug_info",value:function(n){try{var e=i.c.value-16;i.c.value=e,i.l(e,this.ptr,n);var r=g()[e/4+0],t=g()[e/4+1];return s(r,t)}finally{i.c.value+=16,i.d(r,t)}}},{key:"count_agents",value:function(){return i.h(this.ptr)>>>0}}],[{key:"__wrap",value:function(e){var r=Object.create(n.prototype);return r.ptr=e,r}},{key:"new",value:function(){var e=i.j();return n.__wrap(e)}}]),n}(),A=function(){return function(n){o===a.length&&a.push(a.length+1);var e=o;return o=a[e],a[e]=n,e}(new Error)},O=function(n,e){var r=y(c(e).stack,i.e,i.f),t=b;g()[n/4+1]=t,g()[n/4+0]=r},T=function(n,e){try{console.error(s(n,e))}finally{i.d(n,e)}},E=function(n){f(n)},D=function(n,e){throw new Error(s(n,e))}}).call(this,r(56)(n))},91:function(n,e,r){"use strict";var t=r.w[n.i];n.exports=t;r(90);t.o()},92:function(n,e,r){"use strict";r.r(e);var t=r(90);r.d(e,"RenderedAgent",(function(){return t.a})),r.d(e,"Universe",(function(){return t.b})),r.d(e,"__wbg_new_59cb74e423758ede",(function(){return t.d})),r.d(e,"__wbg_stack_558ba5917b466edd",(function(){return t.e})),r.d(e,"__wbg_error_4bb6c2a97407129a",(function(){return t.c})),r.d(e,"__wbindgen_object_drop_ref",(function(){return t.f})),r.d(e,"__wbindgen_throw",(function(){return t.g}))}}]);
//# sourceMappingURL=3.511b9458.chunk.js.map