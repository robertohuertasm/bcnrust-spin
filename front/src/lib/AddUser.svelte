<script lang="ts">
  import * as api from './api';
  import type { UserModel } from './userModel';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let users: UserModel[] = [];
  let userName = '';
  let userEmail = '';
  let isValid = false;

  $: {
    const trimmedName = userName.trim();
    const trimmedEmail = userEmail.trim();
    const rxEmail = /\S+@\S+\.\S+/;
    isValid =
      trimmedName.length && trimmedEmail.length && rxEmail.test(trimmedEmail);
  }

  async function refreshUsers() {
    users = await api.getUsers();
    dispatch('winner', undefined);
  }

  async function getWinner() {
    const winner = await api.getWinner();
    dispatch('winner', winner);
  }

  async function createUser() {
    const user = { name: userName, email: userEmail };
    const newUser = await api.createUser(user);
    if (newUser) {
      users = [...users, newUser].sort((a, b) => a.name.localeCompare(b.name));
    }
  }

  function controlEnter(event) {
    console.log(event.key);
    if (event.key === 'Enter') {
      createUser();
    }
  }
</script>

<div class="container">
  <div>
    <input
      type="text"
      bind:value={userName}
      placeholder="Enter a name"
      id="name"
    />
    <input
      type="text"
      bind:value={userEmail}
      on:click={controlEnter}
      placeholder="Enter an email"
      id="email"
    />
  </div>
  <div class="buttons">
    <button class="winner" on:click={getWinner}> Get Winner </button>
    <button on:click={refreshUsers}> Refresh Users </button>
    <button class="create" on:click={createUser} disabled={!isValid}>
      + Add User
    </button>
  </div>
</div>
<hr />

<style>
  .container {
    display: flex;
    flex-direction: row;
    justify-content: start;
    align-items: center;
    margin: 0;
  }

  .buttons {
    margin-left: auto;
  }
  input {
    padding: 0.8em;
    margin: 0 0.5em;
    border: 1px solid #ccc;
    border-radius: 0.25em;
  }
  button {
    margin: 0 0.4em;
    /* margin-left: auto; */
    align-self: flex-end;
    padding: 0.5em;
    border: 1px solid #ccc;
    border-radius: 0.25em;
    background: #fff;
    cursor: pointer;
    float: right;
  }
  button:disabled {
    cursor: not-allowed;
    color: #ccc;
  }
  button:hover {
    filter: drop-shadow(0 0 2em #d20303aa);
    background-color: #d20303aa;
    color: white;
  }
  button.winner:hover {
    filter: drop-shadow(0 0 2em #d6b406aa);
    background-color: #d6b406aa;
    color: white;
  }
  button.create:hover {
    filter: drop-shadow(0 0 2em #066ed6aa);
    background-color: #066ed6aa;
    color: white;
  }
  button:disabled:hover {
    filter: none;
    background-color: white;
    color: #ccc;
  }
</style>
