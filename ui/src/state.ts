export default interface State {
  notification?: {
    text: string;
    id: number;
  };
  text: string;
  filePath: string;
}
