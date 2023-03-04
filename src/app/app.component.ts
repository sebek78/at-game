import { Component } from '@angular/core';
import { invoke } from '@tauri-apps/api';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  count = 0;
  action = '';

  increase() {
    this.count += 1;
    this.action = '';
  }

  reset() {
    this.count = 0;
    this.action = '';
  }

  load() {
    invoke<string>('load_from_file').then((response: string) => {
      this.count = parseInt(response, 10);
      this.action = 'loaded';
    });
  }
  save() {
    invoke('save_to_file', { data: this.count.toString() }).then((response) => {
      console.log(response);
      this.action = 'saved';
    });
  }
}
