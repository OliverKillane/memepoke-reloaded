import requests
import requests.auth

client_id = "WI5Q-r9Nu3EjSxIjz5SzEA"
redirect_uri = "http://127.0.0.1:8080/memepoke"
about_uri = "http://127.0.0.1:8080/about"

def get_token_m(code):
    return requests.post("https://www.reddit.com/api/v1/access_token",
        headers = {"Authorization" : "Basic " + client_id + ":","User-agent" : "Memepoke/2.0 Running from actix web server",},
        data = {"grant_type" : "authorization_code","code" : code,"redirect_uri" : redirect_uri}
        )

def get_token(code):
	client_auth = requests.auth.HTTPBasicAuth(client_id, "")
	post_data = {"grant_type": "authorization_code",
				 "code": code,
				 "redirect_uri": redirect_uri}
	return requests.post("https://ssl.reddit.com/api/v1/access_token",
							 auth=client_auth,
							 data=post_data,
                             headers = {'User-agent': "Memepoke/2.0 Running from actix web server"})


def get_me(auth):
    return requests.get("https://oauth.reddit.com/api/v1/me", headers = {"User-agent" : "Memepoke/2.0 Running from actix web server", "Authorization" : "Bearer " + auth})

#1073296048264-2L4zdYKoiulV38gvaI4OM7pMvoHtGQ