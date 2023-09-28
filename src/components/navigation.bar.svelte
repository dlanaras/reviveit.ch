<script>
  import NavigationElement from "./navigation.element.svelte";

  export let hidden = false;
  export let isMobile = false;
</script>

<div class="{hidden ? 'hide' : ''}{isMobile ? ' nav-mobile' : ''}">
  <NavigationElement />
  <NavigationElement path="/offers" name="Lager" />
  <NavigationElement path="/contact" name="Kontakt" />
  <NavigationElement path="/news" name="Neuigkeiten" />
  <NavigationElement path="/about" name="Über uns" />
</div>

<style>
  div {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: #800080;
    flex-direction: row;
    text-align: center;
    width: 100%;
    z-index: 1;
    position: relative;
  }

  .hide {
    display: none;
  }

  .nav-mobile {
    flex-direction: column;
    height: 100%;
  }
</style>