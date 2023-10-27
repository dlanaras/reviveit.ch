<div class="container">
    <img src="authorized_only.png" alt="AUTHORIZED PERSONAL ONLY">
    <form>
        <label for="Username">Name</label>
        <input type="text" name="name">
        <label for="Password">Password</label>
        <input type="password" name="password">
        <input type="submit" value="Authorize">
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
</style>