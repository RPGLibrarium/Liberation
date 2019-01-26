const API = axios.create({
    baseURL: 'http://localhost:8080/v1/',
    timeout: 1000,
    responseType:'json',
});
// inject auth header if not already set and a token is available
// API.interceptors.request.use (
//   config => {
//     if(!config.headers.Authorization && keycloak && keycloak.authenticated){
//       config.headers.Authorization = `Bearer ${keycloak.token}`;
//     }
//     return config;
//   },
//   error => Promise.reject(error)
// );

const TEMPLATES = {};

// testing in progrss
document.addEventListener("DOMContentLoaded", initPage);

function initPage(){
  loadTemplates().then(()=>loadStuff());
  // loadStuff();
}

function loadTemplates(){
  const loadTpl = name => axios(`templates/${name}.mustache`)
    .then(res => {
      TEMPLATES[name] = res.data;
      Mustache.parse(TEMPLATES[name]);
    });
  return axios.all([
    loadTpl('rpg_systems_list'),
    loadTpl('titles_list'),
  ])
    .catch(err => console.error('something went wrong (fetching templates)', err));
}

function loadStuff(){
  // rpg systems
  API({
      method: 'GET',
      url: '/rpgsystems',
  })
    .then(stuff => {
      let rendered = Mustache.render(TEMPLATES.rpg_systems_list, stuff.data);
      console.log('rendered', rendered);
      let section = document.createElement('section');
      section.classList.add('content');
      section.innerHTML = rendered;
      document.querySelector('main').appendChild(section);
    })
    .catch(err => console.error('we got error'));

    // titles
    API({
        method: 'GET',
        url: '/titles',
    })
      .then(stuff => {
        let rendered = Mustache.render(TEMPLATES.titles_list, stuff.data);
        console.log('rendered', rendered);
        let section = document.createElement('section');
        section.classList.add('content');
        section.innerHTML = rendered;
        document.querySelector('main').appendChild(section);
      })
      .catch(err => console.error('we got error'));
}
