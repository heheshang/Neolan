from openai import OpenAI

client = OpenAI(
  base_url = "https://integrate.api.nvidia.com/v1",
  api_key = "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"
)
#   model="minimaxai/minimax-m2",
completion = client.chat.completions.create(

  model="z-ai/GLM",
  messages=[{"content":"你好","role":"user"}],
  temperature=1,
  top_p=0.95,
  max_tokens=8192,
  stream=True
)

for chunk in completion:
  if chunk.choices[0].delta.content is not None:
    print(chunk.choices[0].delta.content, end="")
  

