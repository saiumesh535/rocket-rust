import { Component, OnDestroy, ChangeDetectorRef } from '@angular/core';
import { Subject, BehaviorSubject } from 'rxjs';
import { LoginHttpService, BASE_URL } from 'src/http_services/login.http.service';
import { takeUntil } from 'rxjs/operators';
import { HttpClient } from '@angular/common/http';


interface Users {
  username: string;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnDestroy {
  public username = 'saiumesh';
  public password = 'saiumesh';
  public token = '';
  public users = new BehaviorSubject<Users[]>([]);
  private ngTakeUntil = new Subject();

  constructor(private loginHttp: LoginHttpService, private http: HttpClient) {
    this.loginHttp.onLoginSuccess.pipe(
      takeUntil(this.ngTakeUntil)
    )
    .subscribe(this.onLoginSuccess.bind(this));
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

  public getUsers(): void {
    this.http.get<{ users: Users[] }>(`${BASE_URL}users`).subscribe((data) => {
      this.users.next(data.users);
    });
  }

  public copyToken(tokenElement: HTMLInputElement): void {
    tokenElement.select();
    document.execCommand('copy');
  }

  private onLoginSuccess(token: string): void {
    this.token = token;
  }


}
