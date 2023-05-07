<script lang="ts">
  import svelteLogo from './assets/svelte.svg';
  import viteLogo from '/vite.svg';
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
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <h1>Vite + Svelte</h1>

  <AddUser bind:users on:winner={handleWinner} />

  {#each users as user (user.email)}
    <ul>
      {#if winner && winner.email === user.email}
        <li>Winner!! <strong><User {user} /></strong></li>
      {:else}
        <li><User {user} /></li>
      {/if}
    </ul>
  {:else}
    <!-- this block renders when users.length === 0 -->
    <p>No users available</p>
  {/each}
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
</style>
