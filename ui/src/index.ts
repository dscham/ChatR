import { invoke } from '@tauri-apps/api';

class Main {
  constructor() {
      let peerListThread = setInterval(this.updatePeerList, 1000);


  }

  updatePeerList() {
      invoke('get_peer_list').then(console.log);
  }
}

let main = new Main();