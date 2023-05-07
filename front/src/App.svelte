<script lang="ts">
  import ferrisLogo from '/ferris.png';
  import fermyonLogo from '/fermyon.png';
  import AddUser from './lib/AddUser.svelte';
  import User from './lib/User.svelte';
  import { onMount } from 'svelte';
  import type { UserModel } from './lib/userModel';
  import { getUsers } from './lib/api';

  let users: UserModel[] = [];
  let winner: UserModel | undefined;

  function handleWinner(event: CustomEvent<UserModel>) {
    winner = event.detail;
  }

  onMount(async () => {
    users = await getUsers();
  });
</script>

<main>
  <div>
    <a
      href="https://https://www.rust-lang.org/"
      target="_blank"
      rel="noreferrer"
    >
      <img src={ferrisLogo} class="logo svelte" alt="Ferris Logo" />
    </a>
    <a href="https://www.fermyon.com" target="_blank" rel="noreferrer">
      <img src={fermyonLogo} class="logo" alt="Fermyon Logo" />
    </a>
  </div>
  <h1>Win your Ferris!</h1>

  <AddUser bind:users on:winner={handleWinner} />

  <ul>
    {#each users as user (user.email)}
      <li class:winner={winner && winner.email === user.email}>
        <User {user} />
      </li>
    {:else}
      <!-- this block renders when users.length === 0 -->
      <li>No users available</li>
    {/each}
  </ul>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .winner {
    background-color: #ffff00ec;
    filter: drop-shadow(0 0 1em #eeea24aa);
    color: #81371bf2;
  }
  ul {
    list-style-type: none;
    margin: auto;
    display: block;
  }
  li {
    margin: 5px;
    padding: 5px;
    float: left;
    min-width: 22%;
    border: 1px solid #ccc;
  }
</style>
