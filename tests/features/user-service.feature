@user-tests
Feature: Test Connected Home User Service API

  Background:
    Given I set API url to "http://127.0.0.1:8082/api"


  @register-user-pos-1
  Scenario: Register new user
    Given I Set "POST" api endpoint to "register"
    When I Set HEADER param request content type as "application/json"
    And I set request Body like in "/../requests/user-service/register.json"
    And I update request body key "username" from context
    And I update request body key "password" from context
    And I send HTTP POST request
    Then I receive HTTP response code "201"
    And Response BODY is non-empty
    And JSON schema should be like in "/../responses/user-service/register_positive.json"
    And I save token in context for future requests


  @register-user-neg-1
  Scenario: Register existing user
    Given I Set "POST" api endpoint to "register"
    When I Set HEADER param request content type as "application/json"
    And I set request Body like in "/../requests/user-service/register.json"
    And I update request body key "username" from context
    And I update request body key "password" from context
    And I send HTTP POST request
    Then I receive HTTP response code "400"
    And Response BODY is non-empty
#    And JSON schema should be like in "/../responses/user-service/register_negative.json"


  @login-pos-1
  Scenario: Login with existing user
    Given I Set "POST" api endpoint to "login"
    When I Set HEADER param request content type as "application/json"
    And I set request Body like in "/../requests/user-service/login.json"
    And I update request body key "username" from context
    And I update request body key "password" from context
    And I send HTTP POST request
    Then I receive HTTP response code "200"
    And Response BODY is non-empty
    And JSON schema should be like in "/../responses/user-service/login.json"


  @login-neg-1
  Scenario: Login with non-existent user
    Given I Set "POST" api endpoint to "login"
    When I Set HEADER param request content type as "application/json"
    And I set request Body like in "/../requests/user-service/login.json"
    And I update request body key "username" to random value
    And I update request body key "password" to random value
    And I send HTTP POST request
    Then I receive HTTP response code "400"
    And Response BODY is non-empty
#    And JSON schema should be like in "/../responses/user-service/login.json"


  @profile-pos-1
  Scenario: Retrieve user profile
    Given I Set "GET" api endpoint to "profile"
    When I Set HEADER param request content type as "application/json"
    When I Set HEADER param request authorization from context
    And I set request Body like in "/../requests/user-service/profile.json"
    And I send HTTP GET request
    Then I receive HTTP response code "200"
    And Response BODY is non-empty
    And JSON schema should be like in "/../responses/user-service/profile.json"