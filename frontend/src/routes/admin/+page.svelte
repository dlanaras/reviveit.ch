<div class="content container">
    <img src="authorized_only.png" alt="AUTHORIZED PERSONAL ONLY">
    <form method="POST" action="?/login">
        <label for="Username">Name</label>
        <input type="text" name="username">
        <label for="Password">Password</label>
        <input type="password" name="password">
        <label for="Register Secret">Register Secret</label>
        <input type="password" name="register_secret">
        <div class="buttons">
            <input type="submit" value="Login">
            <input type="submit" value="Register" formaction="?/register">
        </div>
    </form>
</div>

<style>
    form {
        display: flex;
        flex-direction: column;
        width: 80%;
        min-width: 20%;
        align-items: center;
    }

    form input {
        color: black;
        width: 13rem;
    }

    input[type="submit"] {
        width: 8rem;
        margin-top: 1em;
    }

    img {
        width: 50%;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .buttons {
        display: flex;
        justify-content: space-evenly;
    }
</style>