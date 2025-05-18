const information = document.getElementById("info")
information.innerText = `this app is using Chrome (v${versions.chrome()})`

const func = async () => {
  const response = await window.versions.ping()
  console.log(response)
}

func()
