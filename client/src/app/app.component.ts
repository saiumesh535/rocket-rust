import { Component, OnDestroy, ChangeDetectorRef } from '@angular/core';
import { Subject } from 'rxjs';
import { LoginHttpService } from 'src/http_services/login.http.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnDestroy {
  public username = 'saiumesh';
  public password = 'saiumesh';
  public token: string = '';
  private ngTakeUntil = new Subject();

  constructor(private loginHttp: LoginHttpService) {
    this.loginHttp.onLoginSuccess.subscribe(this.onLoginSuccess.bind(this));
  }

  public ngOnDestroy(): void {
    this.ngTakeUntil.next();
    this.ngTakeUntil.complete();
  }

  private login(): void {
    this.loginHttp.loginService.next({
      username: this.username,
      password: this.password
    });
  }

  private onLoginSuccess(token: string): void {
    this.token = token;
  }

  public copyToken(tokenElement: HTMLInputElement): void {
    tokenElement.select();
    document.execCommand('copy');
  }

}
