from behave import given, when, then, step

import os
import requests
import json
import jsonschema
import random
import string


@given(u'I set API url to "{url}"')
def step_impl(context, url):
    context.api_base_url = url


@when(u'I Set HEADER param request content type as "{header_content_type}"')
def step_impl(context, header_content_type):
    context.headers['Content-Type'] = header_content_type


@when(u'I Set HEADER param request authorization from context')
def step_impl(context):
    context.headers['Authorization'] = 'Bearer ' + context.feat['token']


@then(u'I save token in context for future requests')
def step_impl(context):
    context.feat['token'] = json.loads(context.response.text)['token']


@when(u'I update request body key "{key}" from context')
def step_impl(context, key):
    context.params[key] = context.feat[key]


@when(u'I update request body key "{key}" to random value')
def step_impl(context, key):
    context.params[key] = ''.join(random.choice(string.ascii_lowercase) for i in range(8))


@when(u'I set request Body like in "{request_body_file}"')
def step_impl(context, request_body_file):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    with open(dir_path + request_body_file, 'r', encoding='UTF-8') as file_contents:
        data = file_contents.read()
    json_body = json.loads(data)
    context.params = json_body


@given(u'I Set "{method}" api endpoint to "{endpoint}"')
def step_impl(context, method, endpoint):
    key = method + '_URL'
    context.api_endpoint = context.api_base_url + '/' + endpoint


@when(u'I send HTTP POST request')
def step_impl(context):
    context.response = requests.post(url=context.api_endpoint, json=context.params,
                                     headers=context.headers)


@when(u'I send HTTP GET request')
def step_impl(context):
    context.response = requests.get(url=context.api_endpoint, headers=context.headers)


@then(u'I receive HTTP response code "{response_code}"')
def step_impl(context, response_code):
    assert str(context.response.status_code) == response_code


@then(u'Response BODY is non-empty')
def step_impl(context):
    assert context.response.text is not None


@then(u'Response BODY "{request_name}" is empty')
def step_impl(context, request_name):
    assert context.response.text is None


@step('JSON schema should be like in "{filename}"')
def json_schema_as_in_file(context, filename):
    dir_path = os.path.dirname(os.path.realpath(__file__))
    with open(dir_path + filename, 'r', encoding='UTF-8') as json_schema:
        data = json_schema.read()
    json_body = json.loads(context.response.text)
    jsonschema.validate(json_body, json.loads(data))
