<div class="content">
    <form method="POST" class="column">
        <label for="title">Titel</label>
        <input type="text" name="title" autocomplete="off">
        <label for="content">Inhalt</label>
        <input type="text" name="content" autocomplete="off">
        <label for="date">Datum</label>
        <input type="date" name="date">
        <input type="submit" value="Erstellen">
    </form>
</div>

<style>
    .column {
        display: flex;
        flex-direction: column;
    }

    .column input {
        color: black;
    }
</style>