<script lang="ts">
  import type { UserModel } from './userModel';
  export let user: UserModel;
</script>

<div>
  <p><b>{user.name}</b></p>
  <p>{user.email}</p>
</div>

<style>
  p {
    margin: 0;
    line-height: 30px;
  }
</style>
