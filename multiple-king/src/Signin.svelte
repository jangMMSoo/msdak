<script>
    export let handleSignIn;
</script>

<main>
    <div className="panel-landing" id="section-1">
        <h1 className="landing-heading">Hello, Blockstack!</h1>
        <p className="lead">
            <button
                    className="btn btn-primary btn-lg"
                    id="signin-button"
                    on:cliek={handleSignIn}>
                Sign In with Blockstack
            </button>
        </p>
    </div>
</main>