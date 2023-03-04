import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  title = 'at-game';
  constructor() {
    console.log('at-game');

    invoke('greet', { name: 'World' }).then((response) =>
      console.log(response)
    );
  }
}
