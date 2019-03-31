import { Injectable } from "@angular/core";
import { Subject, Observable, throwError } from 'rxjs';
import { catchError, retry } from 'rxjs/operators';
import { HttpClient, HttpParams, HttpHeaders, HttpErrorResponse } from '@angular/common/http';


export interface Login {
    username: string;
    password: string;
}

export const BASE_URL = 'http://localhost:8000/';

@Injectable()
export class LoginHttpService {
    public onLogin = new Subject<Login>();
    public onLoginSuccess = new Subject<string>();

    constructor(private http: HttpClient) {
        this.onLogin.subscribe(this.loginHttpRequest.bind(this));
    }

    public get loginService(): Subject<Login> {
        return this.onLogin;
    }

    private loginHttpRequest(login: Login): void {
        const body = new HttpParams()
            .set('username', login.username)
            .set('password', login.password);
        this.http.post<{ token: string }>(`${BASE_URL}login`, body.toString(), {
            headers: new HttpHeaders()
                .set('Content-Type', 'application/x-www-form-urlencoded')
        }).pipe(
            catchError(this.handleError)
        ).subscribe((result: { token: string }) => {
            this.onLoginSuccess.next(result.token);
        });
    }

    private handleError(error: HttpErrorResponse): any {
        if (error instanceof HttpErrorResponse) {
            alert(error.error.text);
        }
        if (error.error instanceof ErrorEvent) {
            // A client-side or network error occurred. Handle it accordingly.
            console.error('An error occurred:', error.error.message);
        } else {
            // The backend returned an unsuccessful response code.
            // The response body may contain clues as to what went wrong,
            console.error(
                `Backend returned code ${error.status}, ` +
                `body was: ${error.error}`);
        }
        // return an observable with a user-facing error message
        return throwError(
            'Something bad happened; please try again later.');
    }
}
