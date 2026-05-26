import{bW as v,i as a,b4 as b,bO as j,bD as y,bf as f,bF as o,b0 as e,k as n,T as k}from"./index-DWqfDtQb.js";import{c,b as S,u as _,F as A}from"./createReactComponent-CHQ4yR5-.js";/**
 * @license @tabler/icons-react v3.44.0 - MIT
 *
 * This source code is licensed under the MIT license.
 * See the LICENSE file in the root directory of this source tree.
 */const w=[["path",{d:"M5 13a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v6a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-6",key:"svg-0"}],["path",{d:"M11 16a1 1 0 1 0 2 0a1 1 0 0 0 -2 0",key:"svg-1"}],["path",{d:"M8 11v-4a4 4 0 1 1 8 0v4",key:"svg-2"}]],F=c("outline","lock","Lock",w);/**
 * @license @tabler/icons-react v3.44.0 - MIT
 *
 * This source code is licensed under the MIT license.
 * See the LICENSE file in the root directory of this source tree.
 */const M=[["path",{d:"M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0",key:"svg-0"}],["path",{d:"M6 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2",key:"svg-1"}]],N=c("outline","user","User",M),R="_login_1yea9_1",T="_form_1yea9_11",i={login:R,form:T};function I(){const r=v(),l=a.useNavigate(),{redirect:u}=a.useSearch(),d=S({mutationFn:b}),{AppForm:m,AppField:s,SubmitButton:p,handleSubmit:g}=_({defaultValues:{account:"",password:""},validators:{onSubmit:f({account:o().nonempty("请输入账号"),password:o().nonempty("请输入密码")})},async onSubmit({value:t}){const{accessToken:h}=await d.mutateAsync(t);j.setState(x=>({...x,isAuthenticated:!0,credentials:{accessToken:h}})),await r.invalidate(),await l({to:u,replace:!0}),y({title:"登录成功",message:"👏 欢迎使用Axum Rust构建的系统"})}});return e.jsx("div",{className:i.login,children:e.jsxs(n,{className:i.form,gap:"xl",children:[e.jsx(k,{c:"blue",my:"xl",order:2,ta:"center",children:"基于 Rust+Axum 的起步系统"}),e.jsx(A,{onSubmit:g,children:e.jsxs(n,{gap:"lg",children:[e.jsx(s,{name:"account",children:({TextField:t})=>e.jsx(t,{leftSection:e.jsx(N,{}),placeholder:"账号",radius:"md",size:"lg"})}),e.jsx(s,{name:"password",children:({PasswordField:t})=>e.jsx(t,{leftSection:e.jsx(F,{}),placeholder:"密码",radius:"md",size:"lg"})}),e.jsx(m,{children:e.jsx(p,{fullWidth:!0,mt:"lg",radius:"md",size:"lg",children:"登 录"})})]})})]})})}export{I as component};
