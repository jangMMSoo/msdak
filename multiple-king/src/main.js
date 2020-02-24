import MultiKing from './MultiKing.svelte';
// import App from './App.svelte';
import Signin from './Signin.svelte';
import {blockstack}  from 'blockstack';

const appConfig = new blockstack.AppConfig();
const userSession = new blockstack.UserSession({ appConfig: appConfig });

const app = new Signin({
	target: document.body,
	props :{
		handleSignIn : function (e) {
			e.preventDefault();
			userSession.redirectToSignIn();
		}
	}
});

// const app = new MultiKing({
// 	target: document.body,
// 	props: {
// 		name: 'world'
// 	}
// });

// export default signin;
export default app;