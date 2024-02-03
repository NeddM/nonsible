```bash
            __                    _  _      _
         /\ \ \ ___   _ __   ___ (_)| |__  | |  ___
 _____  /  \/ // _ \ | '_ \ / __|| || '_ \ | | / _ \ _____
|_____|/ /\  /| (_) || | | |\__ \| || |_) || ||  __/|_____|
       \_\ \/  \___/ |_| |_||___/|_||_.__/ |_| \___|
```

# English

Nonsible is an alternative to Ansible. It is open source and developed in **Rust**. The idea is not to depend on factors like Python, as it commits us to having Python installed on all of our machines, among other requirements.

On several occasions, I have encountered the problem that I couldn't install _pip_, and I had to resort to using _pip3_ or even _python3 -m_. This seems very unintuitive to me; every time I install Ansible, I have to do a different workaround.

## Nonsible pros

1. It's cross-platform.
   The project is developed in **Rust**, which allows creating a build independently of the operating system you are working on.

2. Perfect for small tasks.
   This project works very well for installing a package on multiple machines or updating all systems in our company.

3. It's scalable.
   We can organize the **connections** and **tasks** in **2 YAML files**. This allows us to have multiple YAML files depending on the task we want to perform on the machines we want to perform it on. It all depends on how you want to organize yourself!

4. Adapts to your needs.
   Nonsible can be used in various ways. With a _fully interactive interface_, a _semi-interactive interface_, or in a _completely unattended manner_.

## Usage Types

Usage types are _completely interactive_, _semi-interactive_, and _unattended_.

-   **Interactive** (Without any arguments): In the interactive way, we need to manually add the connections, although we can open a YAML file of tasks. This way of running Nonsible is very useful when the task you need to perform is simple or occasional.

-   **Semi-Interactive** (With one argument): The argument we add will be a YAML file of targets (connections). The idea is to load multiple connections, as it can be the most tedious task to perform, and from there, we can install either imperatively or declaratively. It's interesting because we can see in the table the added connections and their details, ideal for checking if all the data is as desired.

-   **Unattended** (With two or more arguments): It is the perfect methodology for creating automation in a CI/CD pipeline. The first argument would be a YAML file of targets (connections), and the second would be the YAML file of tasks. This way of using the script will take care of performing the operating system test and following step by step all the tasks indicated in the YAML file of tasks.

## YAML examples

-   Target YAML: In this YAML we are going to write our connections data as an array.

```yaml
# Without label
- name: ne
  username: nedd
  ip: 1.2.4.8
  sudo: true
  sudo_password: holamundo
  pem: ./pem/pemname2.pem

# With label
- name: illo
  username: hola
  ip: 8.4.2.1
  sudo: true
  sudo_password: thisisapassword
  pem: ./pem/pemname1.pem
  labels:
    - tree
```

-   Tasks YAML: In this YAML we define the tasks as an array.

```yaml
# Install a package, introducing it's name
- name: Install tree
  task: Install
  package: tree

# Uninstall a package, introducing it's name
- name: Uninstall tree
  task: Uninstall
  package: tree

# Simply runs a command
- name: Simply run 1
  task: Run
  command: apt install tree -y

# Update all the system dependencies
- name: UpdateAll
  task: UpdateAll
  package: 

# Upgrade all the system dependencies
- name: UpgradeAll
  task: UpgradeAll
  package: 

# Any of the previous tasks can be labeled as you want
- name: Update with label
  task: UpdateAll
  package: 
  matchLabels: 
    - testing

- name: Upgrade with label
  task: UpgradeAll
  package: 
  matchLabels: 
    - testing

- name: Install tree with label
  task: Install
  package: tree
  matchLabels: 
    - prueba
    - testing

- name: Uninstall tree with label
  task: Uninstall
  package: tree
  matchLabels: 
    - testing
```

## Labels and matchlabels
If you set a matchlabel to a task, the task only will be executed by a connection that has the same label.
This allow you to filtering the execution of tasks.

## Additional arguments
- --help or -h: Print inline help.
- --force: Runs Nonsible even if a connection is failed. The failed connection's tasks wont be executed.
- --continueonerror: Runs Nonsible even if a connection is failed, and executes every task on failed connections too.
- --no-color: Print information about the tasks, CAREFUL! This argument maybe print sensible data.

I will soon provide documentation with all the types of tasks that can be performed, and we will implement more tasks that I believe are necessary.

## Github Action

Nonsible can also be executed through a Github Actions workflow. Here we have an execution example, where I have all the YAML files (the targets file and the tasks file) in a repository, along with the SSH keys.

It's important to change the permissions of the SSH keys using `chmod 400`, as shown in the example.

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout del código
      uses: actions/checkout@v3

    - name: Chmod the keys
      run: |
        chmod 400 ./pem/piensa
        chmod 400 ./pem/raspi

    - name: Execute Nonsible
      uses: NeddM/nonsible@v0.1
      with:
        # Required arguments
        targetYAML: targets/targets1.yaml
        taskYAML: tasks/tasks1.yaml
        # The arguments bellow are optional
        continueonerror: false
        force: true
        nocolor: false
```

---

# Español

Nonsible es una alternativa a Ansible. Es open source y desarrollada en **Rust**. La idea es no depender de factores como Python, ya que nos compromete a tener instalado Python en todas nuestras máquinas, entre otros requisitos.

En varias ocasiones me he encontrado con el problema de que no podía instalar _pip_, y tuve que tomar la alternativa de usar _pip3_ o incluso _python3 -m_. Esto me resulta muy poco intuitivo, cada vez que instalo Ansible tengo que hacer una bilguería distinta.

## Ventajas de Nonsible

1. Es multiplataforma.
   El proyecto está desarrollado en **Rust**, lo cual permite crear una build independientemente del sistema operativo en el que estemos trabajando.

2. Perfecto para pequeñas tareas.
   Este proyecto funciona muy bien para instalar un paquete en varios equipos, o actualizar todos los sistemas de nuestra empresa.

3. Es escalable.
   Podemos organizar las **conexiones** y las **tareas** en **2 archivos YAML**. Esto nos permite tener varios archivos YAML dependiendo de la tarea que queramos realizar en los equipos que queramos realizarlo. ¡Todo depende de como te quieras organizar tu mismo!.

4. Se adapta a tus necesidades.
   Nonsible puede usarse de varias maneras. Con una interfaz _completamente interactiva_, una interfaz _semi interactiva_, o de manera totalmente _desatendida_.

## Tipos de uso

Los tipos de uso son _completamente interactiva_, _semi interactiva_ y _desatendida_.

- Interactiva (Sin ningún argumento): En la manera interactiva tenemos que añadir nosotros las conexiones manualmente, aunque sí que podemos abrir un archivo YAML de tareas. Esta manera de ejecutar el Nonsible es muy útil cuando la tarea que tienes que realizar es sencilla o puntual.

- Semi-Interactiva (Con un argumento): El argumento que añadimos será un archivo YAML de targets (conexiones). La idea es cargar varias conexiones, ya que puede ser la tarea más tediosa de realizar, y a partir de ahí ya podemos instalar de manera imperativa o declarativa. Es interesante porque podemos ver en la tabla las conexiones agregadas y sus detalles, ideal para comprobar si todos los datos están como deseamos.

- Desatendida (Con dos argumentos): Es la metodología perfecta para crear una automatización en un pipeline de CICD. El primer argumento sería un archvio YAML de targets (conexiones), y el segundo sería el archivo YAML de tareas. Esta manera de usar el script se encargará de realizar el test de sistema operativo, y de seguir paso a paso todas las tareas que se le indiquen en el YAML de tareas.

## Ejemplos de YAMLs.
- Target YAML (conexiones): En este YAML vamos a definir los datos de nuestras conexiones. La idea es definirlos a modo de array.

```yaml
# Sin etiqueta
- name: ne
  username: nedd
  ip: 1.2.4.8
  sudo: true
  sudo_password: holamundo
  pem: ./pem/pemname2.pem

# Con etiqueta
- name: illo
  username: hola
  ip: 8.4.2.1
  sudo: true
  sudo_password: thisisapassword
  pem: ./pem/pemname1.pem
  labels:
    - tree
```

- Tasks YAML (tareas): Definimos las tareas que queremos realizar, en el orden deseado. Hay que definir las tareas a modo de array.

```yaml
# Instala un paquete, introduciendo su nombre
- name: Install tree
  task: Install
  package: tree

# Desinstala un paquete
- name: Uninstall tree
  task: Uninstall
  package: tree

# Executa un comando
- name: Simply run 1
  task: Run
  command: apt install tree -y

# Actualiza todas las dependencias del sistema
- name: UpdateAll
  task: UpdateAll
  package: 

# Upgradea todas las dependencias del sistema
- name: UpgradeAll
  task: UpgradeAll
  package: 

# Todas las tareas que hemos visto previamente se pueden etiquetar
- name: Update with label
  task: UpdateAll
  package: 
  matchLabels: 
    - testing

- name: Upgrade with label
  task: UpgradeAll
  package: 
  matchLabels: 
    - testing

- name: Install tree with label
  task: Install
  package: tree
  matchLabels: 
    - prueba
    - testing

- name: Uninstall tree with label
  task: Uninstall
  package: tree
  matchLabels: 
    - testing
```

## Labels y matchlabels
Si configuras una tarea con una matchlabel, esta tarea sólo será ejecutada por una conexión que tenga la misma etiqueta.
Esto te permite filtrar la ejecución de algunas tareas en algunas conexiones.

## Argumentos adicionales
- --help or -h: Imprime la ayuda en pantalla.
- --force: Ejecuta Nonsible incluso de una conexión ha fallado. Las tareas de las conexiones fallidas no serán ejecutadas.
- --continueonerror: Ejecuta Nonsible incluso si la conexión ha fallado, y tamibén ejecuta todas las tareas de esa conexión fallida.
- --no-color: Imprime por pantalla información adicional sobre las tareas. ¡CUIDADO! Este argumento puede que imprima información sensible.

Pronto dejaré lista una documentación con todos los tipos de tareas que se pueden realizar, e implementaremos más tareas que yo pienso que son necesarias.

## Github Action
Nonsible también se puede ejecutar a través un workflow de Github Actions. Aquí tenemos un ejemplo de ejecución, en el que tengo en un repositorio todos los archivos YAML (El archivo de targets y el de tasks), y también las claves SSH.

Es importante que cambiemos los permisos de las claves SSH, con un `chmod 400` como podemos ver en el ejemplo.

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout del código
      uses: actions/checkout@v3

    - name: Chmod the keys
      run: |
        chmod 400 ./pem/piensa
        chmod 400 ./pem/raspi

    - name: Execute Nonsible
      uses: NeddM/nonsible@v0.1
      with:
        # Required arguments
        targetYAML: targets/targets1.yaml
        taskYAML: tasks/tasks1.yaml
        # The arguments bellow are optional
        continueonerror: false
        force: true
        nocolor: false
```
