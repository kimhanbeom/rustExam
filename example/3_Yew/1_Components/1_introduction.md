1.component란?
--------------
 - components는 yew의 구성 블록 단위 입니다.  (단위 요소)
 - 스스로의 상태 관리 및 Dom 객체로 랜더링 할 수 있음
 - Components는 Component trait을 구현하여 생성됩니다.
 - Component trait은 구현해야 할 필요가 있는 여러가지 의 메소드를 가지고 있음
 - Yew은 componets의 lifecycle동안 각각의 다른 단계에서 해당 메소드들을 호출할 것입니다.


2.LifeCycle Method
-----------------
1). Create
 - Component 생성될 떄, 해당 메서드는 부모 componet로부터 설정들을 받고, 그 설정들은 create 메서드를 통해 전달되는 <span style="color:red"> **Context\<Self\>** </span>에 저장됩니다.
 - 그 설정들은 component의 상태 초기화에 사용됩니다.
 - "link"는 콜백의 등록이나 컴포넌트에게 메세지 전달을 위해 사용됩니다.

 ```rust
    use yew::{Component, Context, html, Html, Properties};

    #[derive(PartialEq, Properties)]
    pub struct Props;
    pub struct MyComponent;

    impl Component for MyComponent {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        MyComponent
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl

    }
}
 ```


2). View
 - view 메서드는 유저가 어떻게 component를 dom객체로 랜더링 해야 되는지 작성하는 곳입니다.
 (실제 화면단 구현을 이곳에 표시한다고 생각하면 됩니다.)
 - rust function을 사용하여 HTML 방식으로 코드로 작성하 는것은 꽤 난잡하기 떄문에 Yew은 <span style="color:red"> **html!** </span> 매크로를 제공합니다.
 - 이 매크로는 React's jsx와 유사합니다.
 - 한가지 차이점이 있다면 yew은 properties를 위한 shorthad syntax를 제공합니다.
 ex) onclick={onclick} 대신 {onclick}으로 사용하면 됩니다.

 ```rust
    use yew::{Component, Context, html, Html, Properties};

    enum Msg {
        Click,
    }

    #[derive(PartialEq, Properties)]
    struct Props {
        button_text: String,
    }

    struct MyComponent;

    impl Component for MyComponent {
        type Message = Msg;
        type Properties = Props;

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let onclick = ctx.link().callback(|_| Msg::Click);
            html! {
                <button {onclick}>{ &ctx.props().button_text }</button>
            }
        }
    }
 ```


3). Rendered
- Redered 메서드는 view 메서드가 호출 됬을때 한번만 호출됩니다.
- yew은 브라우저가 페이지 새로고침을 하기전에 Dom에 결과들을 가지고 render 합니다.
- component가 element의 rendering을 완료한 뒤 수행하고자하는 action들을 원할떄 매우 유용합니다.
- first_render 파라미터는 이 메서드가 첫번쨰 랜더나 다음랜더에 중 어떤때에 호출 될지 결정할떄 사용할 수 있습니다.


```rust
    use web_sys::HtmlInputElement;
    use yew::{
        Component, Context, html, Html, NodeRef,
    };

    pub struct MyComponent {
        node_ref: NodeRef,
    }

    impl Component for MyComponent {
        type Message = ();
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                node_ref: NodeRef::default(),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <input ref={self.node_ref.clone()} type="text" />
            }
        }

        fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
            if first_render {
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                    input.focus();
                }
            }
        }
    }
```


* * *
redered 라이프 사이클 메서드는 필수 구현이 아니며,아무것도 수행하지 않는것이 default입니다.
* * *

4). update
- message를 통한 components간의 통신을 다루는 메서드
- 메세지에 따라서 필요로 하면 스스로 re-redering 하여 컴포넌트가 업데이트 될 수 있도록 합니다.
- messages은 이벤트 리스너,자식 components,Agent,Service나 futures에 보낼 수 있습니다.

```rust
    use yew::{Component, Context, html, Html};

    pub enum Msg {
        SetInputEnabled(bool)
    }

    struct MyComponent {
        input_enabled: bool,
    }

    impl Component for MyComponent {
        type Message = Msg;
        type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // Re-render
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // impl
        }
    }

    }
```

5). changed
 - component가 부모 component에 의해 새로운 properties를 전달 받고 re-render 해야될때 changed 메서드가 호출 됩니다.
 (이것은 default로 구현되어 있습니다.)


6). Destroy
 - Dom에서 component가 unmounte 되면, Yew은 destroy 메서드를 호출합니다.
 - 이 메서드의 사용은 필수가 아닌 선택이며 default로 아무 동작 하지 않습니다.

3.Infinite loops
----------------
```rust
    use yew::{Context, Component, Html};

    struct Comp;

    impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // We are going to always request to re-render on any msg
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // For this example it doesn't matter what is rendered
        Html::default()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // Request that the component is updated with this new msg
        ctx.link().send_message(());
    }
    }
```
  1). component가 create function을 통해 생성

  2). view 메서드 호출되서 yew이 브라우저 dom에 랜더링할 내용을
  알게됨.

  3).context link을 사용하여 업데이트 메세지를 예약하여 rendered 메서드를 호출합니다.

  4). yew은 post-reder 차례를 끝냅니다.

  5). yew은 예약된 이벤트와 메세지큐가 비었는지 체크합니다.

  6). component의 re-render필요하고 무언가의 변화됨을 나타내게 되면 true가 리턴되어
      update 메서드가 호출됩니다.

  7).2번으로 점프합니다.

  * * *
  업데이트 예약을 rendered 메서스에서 하는것은 유용하지만 , 이러한 loop를 어ㄴ떻게 끝낼것인지 명시해야합니다.
  * * *



4.Associtated Types
----------------
 - component trait은 2가지 연관된 타입을 가지고 있습니다.(Message /Properties)

   1). Message
   - 이벤트 발생시 component로 메세지 전달시 사용
    ex) 버튼클릭,스크롤다운시 하나이상의 이벤트를 처리시 유용
    ```rust
        enum Msg {
            Click,
            FormInput(String)
        }
    ```

   2). Properties
   - 부모 component로부터 전달된 정보를 표현합니다.
   - 이 타입은 Properties trait을 반드시 구현해야 합니다.
   - component를 create/update 할때 사용됩니다.


5.Context
 - 모든 component의 라이프사이클 메서드들은 context object를 가지고 있습니다.
 - 이 object는 component에게 메세지를 전송하고, component에게 prop를 전달 할
   수있는 component의 scope에 대한 reference를 제공합니다.
