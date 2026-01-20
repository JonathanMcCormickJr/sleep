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


## MVP commands history

```bash
jonathan@jonathan-computer:~$ curl --location 'localhost:8000/question' \ 
--header 'Content-Type: application/json' \
--data '{
    "title": "Newly Created Question",
    "description": "My Description"
}'
{"question_uuid":"a12aed05-b6eb-424f-93f9-2d631e7b29b0","title":"Newly Created Question","description":"My Description","created_at":"2026-01-13 12:27:25.828694"}jonathan@jonathan-computer:~$jonathan@jonathan-computer:~$ curl --location 'localhost:8000/questions'
[{"question_uuid":"a12aed05-b6eb-424f-93f9-2d631e7b29b0","title":"Newly Created Question","description":"My Description","created_atjonathan@jonathan-computer:~$ curl --location --request DELETE 'localhost:8000/question' \LETE 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "[UUID of a created question]"
}'
Something went wrong! Please try again.jonathan@jonathan-ThinkCentjonathan@jonathan-computer:~$ curl --location --request DELETE 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "a12aed05-b6eb-424f-93f9-2d631e7b29b0"
}'
jonathan@jonathan-computer:~$ curl --location 'localhost:8000/questions'
jonathan@jonathan-computer:~$ curl --location 'localhost:8000/question' --header 'Content-Type: application/json' --data '{
    "title": "Newly Created Question",
    "description": "My Description"
}'
{"question_uuid":"38fdb272-f068-4141-8d26-fe9d47850c26","title":"Newly Created Question","description":"My Description","created_at"jonathan@jonathan-computer:~$ curl --location 'localhost:8000/answer' \tion 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "38fdb272-f068-4141-8d26-fe9d47850c26",
    "content": "test question"
}'
{"answer_uuid":"f9493d58-83d3-4263-990d-0716b94df28a","question_uuid":"38fdb272-f068-4141-8d26-fe9d47850c26","content":"test question","created_at":"2026-01-13 12:29:28.542716"}jonathan@jonathan-Thijonathan@jonathan-computer:~$ curl --location --request GET 'localhost:8000/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_uuid": "curl --location --request GET 'localhost:8000/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_uu^C  
}'
jonathan@jonathan-computer:~$ curl --location --request GET 'localhost:8000/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_uuid": "38fdb272-f068-4141-8d26-fe9d47850c26"
}'
[{"answer_uuid":"f9493d58-83d3-4263-990d-0716b94df28a","question_uuid":"38fdb272-f068-4141-8d26-fe9d47850c26","content":"test question","created_at":"2026-01-13 12:29:28.542716"}]jonathan@jonathan-Tjonathan@jonathan-computer:~$ curl --location --request DELETE 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "answer_uuid": "f9493d58-83d3-4263-990d-0716b94df28a"
}'
jonathan@jonathan-computer:~$ curl --location --request GET 'localhost:8000/answers' --header 'Content-Type: application/json' --data '{
    "question_uuid": "38fdb272-f068-4141-8d26-fe9d47850c26"
}'
[]jonathan@jonathan-computer:~$ 
```

## For DB testing 

```bash 
# Stop it if it’s still running
docker stop stack-overflow-db

# Then remove it
docker rm stack-overflow-db
```

```bash
sudo docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres
```
