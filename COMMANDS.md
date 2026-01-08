# Commands

for doing stuff

POST request to `/question` to create a question:

```bash
curl --request POST \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"title": "Title",
	"description": "Description"
  }' \
  --header 'Content-Type: application/json'
```

GET request to `/questions` to get all questions:

```bash
curl --request GET \
  --url http://localhost:8000/questions \
  --header 'Accept: application/json' \
  --header 'Content-Type: application/json'
```

DELETE request to `/question/{id}` to delete a question by ID:

```bash
curl --request DELETE \
  --url http://localhost:8000/question \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}' \
  --header 'Content-Type: application/json'
```

POST request to `/answer` to create an answer:

```bash
curl --request POST \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]",
	"content": "Content"
}' \
  --header 'Content-Type: application/json'
```

GET request to `/answers` to get all answers:

```bash
curl --request GET \
  --url http://localhost:8000/answers \
  --header 'Accept: application/json' \
  --data '{
	"question_uuid": "[UUID of a created question]"
}' \
  --header 'Content-Type: application/json'
```

DELETE request to `/answer` to delete an answer by ID:

```bash
curl --request DELETE \
  --url http://localhost:8000/answer \
  --header 'Accept: application/json' \
  --data '{
	"answer_uuid": "[UUID of a created answer]"
}' \
  --header 'Content-Type: application/json'
```