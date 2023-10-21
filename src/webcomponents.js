import { 
  provideFluentDesignSystem,
  fluentSwitch,
  fluentCard,
  fluentCombobox,
  fluentSelect,
  fluentOption,
  fluentTextField,
  fluentButton,
} from '@fluentui/web-components';

provideFluentDesignSystem()
  .register(
    fluentSwitch(),
    fluentCard(),
    fluentCombobox(),
    fluentSelect(),
    fluentOption(),
    fluentTextField(),
    fluentButton(),
  );