<script>
  import { page } from "$app/stores";

  export let path = "/";
  export let name = "m152";
</script>


<a href={path} class={$page.url.pathname === path ? "active" : ""}>
  {#if (path !== "/")}
  <h1>{name}</h1>
  {:else}
  <img src="reviveit.logo.png" alt="ReviveIT logo" height="128px">
  {/if}
</a>

<style>
  a:hover {
    background-color: #800080;
  }

  a:link {
    text-decoration: inherit;
    color: inherit;
  }

  a:visited {
    text-decoration: inherit;
    color: inherit;
  }

  a {
    width: 100%;
  }

  .active {
    background-color: #800080;
    cursor: pointer;
  }

  img {
    margin-top: 16px;
    margin-bottom: 16px;
    filter: drop-shadow(3px 3px 0 black);
  }
</style>