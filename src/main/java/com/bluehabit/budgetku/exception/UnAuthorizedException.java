package com.bluehabit.budgetku.exception;

import org.springframework.security.core.AuthenticationException;

public class UnAuthorizedException extends AuthenticationException {
    private int statusCode;
    public UnAuthorizedException(int statusCode,String msg) {
        super(msg);
        this.statusCode = statusCode;
    }

    public UnAuthorizedException(String msg) {
        super(msg);
        this.statusCode = 401;
    }

    public int getStatusCode() {
        return statusCode;
    }
}
